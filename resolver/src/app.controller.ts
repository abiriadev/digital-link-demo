import { Controller, Get, Req } from '@nestjs/common'
import { AppService } from './app.service'
import { Request } from 'express'

@Controller()
export class AppController {
	constructor(private readonly appService: AppService) {}

	@Get('*')
	resolve(@Req() { url }: Request): string {
		return this.appService.resolveDigitalLink(url)
	}
}
