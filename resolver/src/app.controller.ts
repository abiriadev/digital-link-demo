import {
	Controller,
	Get,
	Query,
	Redirect,
	Req,
} from '@nestjs/common'
import { AppService, Material, Rdf } from './app.service'
import { Request } from 'express'

@Controller()
export class AppController {
	constructor(private readonly appService: AppService) {}

	@Get('*')
	async resolve(
		// NOTE:: hostname is not necessary. but it is required for now. upstreams DLRS needs to be fixed
		@Req() { url, hostname }: Request,
		@Query('linkType') linkType: string,
	): Promise<
		{ redirect: string } | Array<Material> | Rdf
	> {
		console.log('url:', url)
		if (url === '/.well-known/gs1resolver') {
			return await this.appService.getRdf()
		}

		const fullUrl = `http://${hostname}${url}`

		console.log('linktype:', linkType)

		const res =
			await this.appService.resolveDigitalLink(
				fullUrl,
				linkType,
			)

		if (typeof res == 'string') {
			return {
				redirect: res,
			}
		} else return res
	}
}
