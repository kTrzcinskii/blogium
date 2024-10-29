import { Skeleton } from './ui/skeleton';

const PostCardSkeleton = () => {
    return (
        <div className="flex flex-col space-y-2">
            <div className="flex flex-row items-center space-x-2">
                <Skeleton className="h-12 w-12 rounded-full" />
                <Skeleton className="h-4 w-[150px]" />
            </div>
            <div className="space-y-4">
                <Skeleton className="h-2 w-[270px]" />
                <Skeleton className="h-28 w-full" />
            </div>
        </div>
    );
};

export default PostCardSkeleton;
