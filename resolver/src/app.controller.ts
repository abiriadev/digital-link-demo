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

	@Get('.well-known/gs1resolver')
	async rdf(): Promise<Rdf> {
		return await this.appService.getRdf()
	}

	@Get('*')
	@Redirect(undefined, 307)
	async resolve(
		// NOTE:: hostname is not necessary. but it is required for now. upstreams DLRS needs to be fixed
		@Req() { url, hostname }: Request,
		@Query('linkType') linkType: string,
	): Promise<{ url: string } | Array<Material>> {
		console.log('url:', url)

		const fullUrl = `http://${hostname}${url}`

		const res =
			await this.appService.resolveDigitalLink(
				fullUrl,
				linkType,
			)

		if (typeof res == 'string') {
			return {
				url: res,
			}
		} else return res
	}
}
