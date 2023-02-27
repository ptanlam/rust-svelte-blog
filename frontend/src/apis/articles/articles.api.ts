import type { AxiosResponse } from 'axios';

import type { Article } from '../../models';
import { CustomApi } from '../common';
import type { ListResponse } from '../common/responses';

class ArticlesApi extends CustomApi {
	async getList(): Promise<ListResponse<Article>> {
		const response: AxiosResponse<ListResponse<Article>> | null = await this.tryGet<
			ListResponse<Article>
		>('/articles');

		if (response === null) return { items: [] };
		return response.data;
	}
}

export const articlesApi: ArticlesApi = new ArticlesApi();
