import { env } from '$env/dynamic/public';
import {
	isAxiosError,
	type AxiosInstance,
	type AxiosRequestConfig,
	type AxiosResponse
} from 'axios';
import axios from 'axios';

export abstract class CustomApi {
	#client: AxiosInstance;
	protected get Client(): AxiosInstance {
		return this.#client;
	}

	constructor() {
		this.#client = axios.create({
			baseURL: env.PUBLIC_API_URL,
			headers: {
				'Content-Type': 'application/json',
				timeout: 1000
			}
		});
	}

	protected async tryGet<TData>(
		url: string,
		config?: AxiosRequestConfig
	): Promise<AxiosResponse<TData> | null> {
		try {
			return await this.#client.get<TData>(url, config);
		} catch (error) {
			if (isAxiosError(error)) {
				// TODO: handle axios error
			}

			return null;
		}
	}
}
