import { Injectable } from '@nestjs/common'
import { DigitalLinkWasm } from 'digital-link-rs'

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
	): string | Array<Material> {
		const dl = this.parseDigitalLink(path)

		console.log('dl:', dl?.toJSON())

		if (dl) {
			if (dl.gs1_path_key === 'gtin') {
				const gtin = dl.gs1_path.gtin?.gtin

				if (gtin === undefined)
					throw new Error('unreachable')

				return this.queryProduct(gtin)
			} else {
				console.log('not gtin:', dl.gs1_path_key)

				return 'not supported'
			}
		} else {
			return 'not found'
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
