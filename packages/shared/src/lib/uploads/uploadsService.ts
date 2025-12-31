import type { Upload } from '$lib/uploads/types';
import { InjectionToken } from '@gitbutler/core/context';
import type { HttpClient } from '$lib/network/httpClient';

export const UPLOADS_SERVICE: InjectionToken<UploadsService> = new InjectionToken('UploadsService');

export class UploadsService {
	constructor(private readonly httpClient: HttpClient) {}

	async uploadFile(file: File): Promise<Upload> {
		void this.httpClient;
		void file;
		return await Promise.reject('File uploads are disabled in this build.');
	}
}
