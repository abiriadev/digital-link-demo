import {
	BadRequestException,
	HttpException,
	HttpStatus,
	Injectable,
	NotFoundException,
} from '@nestjs/common'
import { DigitalLinkWasm } from 'digital-link-rs'
import { Redirect } from './redirect.type'
import { PrismaService } from 'nestjs-prisma'
import { readFile } from 'fs/promises'

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

				// Do we have any records for this primary key?
				if (res.length >= 1) {
					const def = res.find(
						resource =>
							resource.relation ===
							'gs1:default',
					)

					if (def === undefined)
						throw new NotFoundException(
							`Err: gtin ${gtin} has matching resources but not have default one.`,
						)

					// Is a value of linkType defined?
					if (
						linkType &&
						(linkType === 'linkset' ||
							res.find(
								r =>
									r.relation === linkType,
							))
					) {
						if (linkType === 'linkset') {
							// TODO: use linkset formatter
							return res
						} else {
							return (
								res.find(
									r =>
										r.relation ===
										linkType,
								) as Material
							).url as Redirect // assert!
						}
					} else {
						// default(s)
						if (
							res.find(
								r =>
									r.relation ===
									'gs1:default*',
							)
						) {
							// todo
							return (
								res.find(
									r =>
										r.relation ===
										'gs1:default*',
								) as Material
							).url as Redirect // assert!
						} else return def.url as Redirect
					}
				} else {
					// TODO: can we redirect to another resolver?

					throw new NotFoundException(
						`item with GTIN ${gtin} not found`,
					)
				}
			} else
				throw new BadRequestException(
					`not supported primary key: ${dl.gs1_path_key}`,
				)
		} else
			throw new BadRequestException(
				'Not a valid GS1 Digital Link',
			)
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
		return (
			await this.prisma.resource.findMany({
				where: {
					gtin,
				},
			})
		).map(
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

	async getRdf(): Promise<Rdf> {
		return JSON.parse(
			(
				await readFile(
					'public/resolver-description.json',
				)
			).toString(),
		) as Rdf
	}
}

export interface Rdf {
	resolverRoot: string
	supportedPrimaryKeys: Array<string>
	name?: string
	linkTypeDefaultCanBeLinkset?: boolean
	supportedContextValuesEnumerated?: Array<string>
	supportsSemanticInterpretation?: boolean
	validatesAIcombination?: boolean
	extension_profile?: string
	dataDump?: string
	jsonLdContextLocation?: string
}
