import { Injectable } from '@nestjs/common'

@Injectable()
export class AppService {
	resolveDigitalLink(url: string): string {
		return url
	}
}
