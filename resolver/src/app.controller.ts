import { Controller, Get, Query, Req } from '@nestjs/common'
import { AppService } from './app.service'
import { Request } from 'express'

@Controller()
export class AppController {
	constructor(private readonly appService: AppService) {}

	@Get('*')
	resolve(
		// NOTE:: hostname is not necessary. but it is required for now. upstreams DLRS needs to be fixed
		@Req() { url, hostname }: Request,
		@Query('linkType') linkType: string,
	): string {
		const fullUrl = `http://${hostname}${url}`

		return this.appService.resolveDigitalLink(fullUrl)
	}
}
