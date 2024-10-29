import { useFetchPosts } from '@/api/queries/useFetchPosts';
import PostCard from './PostCard';
import PostCardSkeleton from './PostCardSkeleton';
import { POST_FETCH_COUNT_LIMIT } from '@/const';
import { AlertCircle } from 'lucide-react';
import { Alert, AlertTitle, AlertDescription } from './ui/alert';

const PostsList = () => {
    const { data, fetchNextPage, hasNextPage, status, isFetchingNextPage } =
        useFetchPosts();

    if (status === 'pending') {
        return (
            <div className="space-y-7 py-7">
                {Array.from(
                    { length: Math.max(POST_FETCH_COUNT_LIMIT / 2, 10) },
                    (_, index) => (
                        <PostCardSkeleton key={index} />
                    ),
                )}
            </div>
        );
    }

    if (status === 'error') {
        return (
            <Alert variant="destructive" className="my-7">
                <AlertCircle className="h-4 w-4" />
                <AlertTitle>Error</AlertTitle>
                <AlertDescription>
                    Something went wrong while fetching posts. Please try again
                    later.
                </AlertDescription>
            </Alert>
        );
    }

    return (
        <div className="space-y-7 py-7">
            {data.pages.map((page) =>
                page.data.map((post) => {
                    return (
                        <PostCard
                            key={`${post.posted_at}-${post.username}`}
                            {...post}
                        />
                    );
                }),
            )}
            {/* TODO: it should be some element that when scrolled to it calls fetchNextPage */}
            {hasNextPage && !isFetchingNextPage && (
                <button onClick={() => fetchNextPage()}>Fetch more</button>
            )}
        </div>
    );
};

export default PostsList;
