import {
	BadRequestException,
	HttpException,
	HttpStatus,
	Injectable,
} from '@nestjs/common'
import { DigitalLinkWasm } from 'digital-link-rs'
import { Redirect } from './redirect.type'

export interface Material {
	url: string
	relation: string
	titile: string
	language?: string
	media?: string
	context?: string
}

@Injectable()
export class AppService {
	resolveDigitalLink(
		path: string,
		linkType: string,
	): Redirect | Array<Material> {
		const dl = this.parseDigitalLink(path)

		console.log('dl:', dl?.toJSON())

		// GS1 element strings all valid?
		if (dl) {
			if (dl.gs1_path_key === 'gtin') {
				const gtin = dl.gs1_path.gtin?.gtin

				if (gtin === undefined)
					throw new Error('unreachable')

				const res = this.queryProduct(gtin)

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

	queryProduct(gtin: string): Array<Material> {
		console.log('query: ', gtin)

		return [
			{
				url: 'https://example.com/',
				relation: 'pip',
				titile: 'Example Title',
				language: 'ko',
				media: 'text/json',
				context: 'KR',
			},
		]
	}
}
