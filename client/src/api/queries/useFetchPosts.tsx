import { API_URL, POST_FETCH_COUNT_LIMIT } from '@/const';
import { useInfiniteQuery } from '@tanstack/react-query';
import axios from 'axios';

interface IFetchPostsResponse {
    status: 'success';
    count: number;
    data: IPostResponse[];
    next_cursor: string | null;
}

export interface IPostResponse {
    username: string;
    content: string;
    posted_at: string;
    post_image_url?: string;
    user_avatar_url?: string;
}

interface IFetchPostsError {
    status: 'error';
    message: string;
}

const fetchPostsUrl = `${API_URL}/api/posts`;

const fetchPosts = async (cursor?: string): Promise<IFetchPostsResponse> => {
    let url = `${fetchPostsUrl}?limit=${POST_FETCH_COUNT_LIMIT}`;
    if (cursor) {
        url = `${url}&cursor=${cursor}`;
    }
    try {
        const response = await axios.get<IFetchPostsResponse>(url);
        return response.data;
    } catch (error) {
        if (axios.isAxiosError(error) && error.response) {
            const fetchError = error.response.data as IFetchPostsError;
            if (fetchError.status === 'error' && fetchError.message) {
                throw fetchError;
            }
        }
        throw new Error('An unexpected error occurred');
    }
};

export const useFetchPosts = () => {
    // TODO: maybe invalidate this query automatically after some time
    return useInfiniteQuery({
        queryKey: ['posts'],
        queryFn: ({ pageParam }) => fetchPosts(pageParam),
        initialPageParam: undefined as string | undefined,
        getNextPageParam: (lastPage) => lastPage.next_cursor,
    });
};
