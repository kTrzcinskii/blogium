import { useFetchPosts } from '@/api/queries/useFetchPosts';
import PostCard from './PostCard';
import PostCardSkeleton from './PostCardSkeleton';
import { POST_FETCH_COUNT_LIMIT } from '@/const';

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
        return <div>TODO: add some error message</div>;
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
