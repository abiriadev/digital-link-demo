import { Injectable } from '@nestjs/common'
import { DigitalLinkWasm } from 'digital-link-rs'

@Injectable()
export class AppService {
	resolveDigitalLink(url: string): string {
		return url
	}

	parseDigitalLink(
		rawUrl: string,
	): DigitalLinkWasm | undefined {
		return DigitalLinkWasm.try_from_str(rawUrl)
	}
}
