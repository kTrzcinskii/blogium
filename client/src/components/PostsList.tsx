import { useFetchPosts } from '@/api/queries/useFetchPosts';
import PostCard from './PostCard';
import PostCardSkeleton from './PostCardSkeleton';
import { POST_FETCH_COUNT_LIMIT } from '@/const';
import { AlertCircle } from 'lucide-react';
import { Alert, AlertTitle, AlertDescription } from './ui/alert';
import { useCallback, useRef } from 'react';

const PostsList = () => {
    const {
        data,
        fetchNextPage,
        hasNextPage,
        status,
        isFetchingNextPage,
        isLoading,
    } = useFetchPosts();

    // https://dev.to/kevin-uehara/2-react-query-infinite-scroll-1mg8
    const observer = useRef<IntersectionObserver>();
    const lastElementRef = useCallback(
        (node: HTMLDivElement) => {
            if (isLoading) {
                return;
            }

            if (observer.current) {
                observer.current.disconnect();
            }

            observer.current = new IntersectionObserver((entries) => {
                if (
                    entries[0].isIntersecting &&
                    hasNextPage &&
                    !isFetchingNextPage
                ) {
                    console.log('fetching new page');
                    fetchNextPage();
                }
            });

            if (node) {
                observer.current.observe(node);
            }
        },
        [fetchNextPage, hasNextPage, isFetchingNextPage, isLoading],
    );

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
                        <div ref={lastElementRef}>
                            <PostCard
                                key={`${post.posted_at}-${post.username}`}
                                {...post}
                            />
                        </div>
                    );
                }),
            )}
            {isFetchingNextPage &&
                Array.from({ length: 3 }, (_, index) => (
                    <PostCardSkeleton key={index} />
                ))}
        </div>
    );
};

export default PostsList;
