import { ICreatePostInput } from '@/components/CreatePostForm';
import { API_URL } from '@/const';
import { useMutation, UseMutationResult } from '@tanstack/react-query';
import axios, { AxiosError } from 'axios';

interface ICreatePostResponse {
    status: 'success';
}

interface ICreatePostError {
    status: 'error';
    message: string;
}

const createPostUrl = `${API_URL}/api/posts/create`;

const createPost = async (
    data: ICreatePostInput,
): Promise<ICreatePostResponse> => {
    const formData = new FormData();
    formData.append('username', data.username);
    formData.append('content', data.content);
    if (data.image && data.image.length > 0) {
        formData.append('image', data.image[0]);
    }
    if (data.avatarUrl) {
        formData.append('avatar_path', data.avatarUrl);
    }

    try {
        const response = await axios.post<ICreatePostResponse>(
            createPostUrl,
            formData,
        );
        return response.data;
    } catch (error) {
        if (axios.isAxiosError(error) && error.response) {
            const fetchError = error.response.data as ICreatePostError;
            if (fetchError.status === 'error' && fetchError.message) {
                throw fetchError;
            }
            if (error.status === 413) {
                const error = {
                    status: 'error',
                    message: 'Uploaded file was too big',
                };
                throw error;
            }
        }
        throw new Error('An unexpected error occurred');
    }
};

export const useCreatePostMutation = (): UseMutationResult<
    ICreatePostResponse,
    AxiosError<ICreatePostError>,
    ICreatePostInput
> => {
    return useMutation<
        ICreatePostResponse,
        AxiosError<ICreatePostError>,
        ICreatePostInput
    >({
        mutationFn: createPost,
    });
};
