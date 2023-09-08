import {
	Controller,
	Get,
	Query,
	Req,
	Res,
} from '@nestjs/common'
import { AppService, LinkSetLd, Rdf } from './app.service'
import { Request, Response } from 'express'

@Controller()
export class AppController {
	constructor(private readonly appService: AppService) {}

	@Get('.well-known/gs1resolver')
	async rdf(): Promise<Rdf> {
		return await this.appService.getRdf()
	}

	@Get('*')
	async resolve(
		// NOTE:: hostname is not necessary. but it is required for now. upstreams DLRS needs to be fixed
		@Req() { url, hostname }: Request,
		@Res() response: Response,
		@Query('linkType') linkType: string,
	): Promise<void> {
		console.log('url:', url)

		const fullUrl = `http://${hostname}${url}`

		const res =
			await this.appService.resolveDigitalLink(
				fullUrl,
				linkType,
			)

		console.log('final res:', res)

		if (typeof res === 'string') {
			console.log('redirect')
			response.redirect(res)
		} else {
			console.log('return linkset!')
			response.json(res)
		}
	}
}
