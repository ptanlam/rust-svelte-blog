import { articlesApi } from '../../apis/articles';
import type { ListResponse } from '../../apis/common/responses';
import type { Article } from '../../models';

/** @type {import('./$types').PageServerLoad} */
export async function load(): Promise<ListResponse<Article>> {
	return await articlesApi.getList();
}
