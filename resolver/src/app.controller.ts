import {
	Controller,
	Get,
	Query,
	Redirect,
	Req,
} from '@nestjs/common'
import { AppService, Material } from './app.service'
import { Request } from 'express'

@Controller()
export class AppController {
	constructor(private readonly appService: AppService) {}

	@Get('*')
	resolve(
		// NOTE:: hostname is not necessary. but it is required for now. upstreams DLRS needs to be fixed
		@Req() { url, hostname }: Request,
		@Query('linkType') linkType: string,
	): { redirect: string } | Array<Material> {
		const fullUrl = `http://${hostname}${url}`

		console.log('linktype:', linkType)

		const res = this.appService.resolveDigitalLink(
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
