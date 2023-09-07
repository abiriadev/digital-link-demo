import {
	BadRequestException,
	HttpException,
	HttpStatus,
	Injectable,
} from '@nestjs/common'
import { DigitalLinkWasm } from 'digital-link-rs'
import { Redirect } from './redirect.type'
import { PrismaService } from 'nestjs-prisma'

export interface Material {
	url: string
	relation: string
	title: string
	language?: string
	media?: string
	context?: string
}

@Injectable()
export class AppService {
	constructor(private prisma: PrismaService) {}

	async resolveDigitalLink(
		path: string,
		linkType: string,
	): Promise<Redirect | Array<Material>> {
		const dl = this.parseDigitalLink(path)

		console.log('dl:', dl?.toJSON())

		// GS1 element strings all valid?
		if (dl) {
			if (dl.gs1_path_key === 'gtin') {
				const gtin = dl.gs1_path.gtin?.gtin

				if (gtin === undefined)
					throw new Error('unreachable')

				const res = await this.queryProduct(gtin)

				console.log('res:', res)

				// Is a value of linkType defined?
				if (linkType) {
					if (linkType === 'linkset') {
						return res
					}
					return res[0].url as Redirect
				} else {
					return res[0].url as Redirect
				}
			} else {
				console.log('not gtin:', dl.gs1_path_key)

				throw new BadRequestException(
					'not supported',
				)
			}
		} else {
			throw new BadRequestException(
				'Not a valid GS1 Digital Link',
			)
		}
	}

	parseDigitalLink(
		rawUrl: string,
	): DigitalLinkWasm | undefined {
		console.log(rawUrl)

		return DigitalLinkWasm.try_from_str(rawUrl)
	}

	async queryProduct(
		gtin: string,
	): Promise<Array<Material>> {
		console.log('query: ', gtin)

		return (await this.prisma.resource.findMany()).map(
			({
				url,
				media,
				title,
				context,
				language,
				relation,
			}) => ({
				url,
				relation,
				title,
				media: media ?? undefined,
				context: context ?? undefined,
				language: language ?? undefined,
			}),
		)
	}
}
