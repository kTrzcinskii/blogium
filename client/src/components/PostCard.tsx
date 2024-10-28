import { IPostResponse } from '@/api/queries/useFetchPosts';
import { format } from 'date-fns';

import {
    Card,
    CardHeader,
    CardTitle,
    CardDescription,
    CardContent,
    CardFooter,
} from './ui/card';
import { Avatar, AvatarFallback } from './ui/avatar';
import { AvatarImage } from '@radix-ui/react-avatar';
import { API_URL } from '@/const';

const PostCard = ({
    username,
    content,
    posted_at,
    post_image_url,
    user_avatar_url,
}: IPostResponse) => {
    const postedAt = format(new Date(posted_at), 'dd-MM-yyyy, HH:mm');
    const avatarUrl = `${API_URL}${user_avatar_url}`;
    const imageUrl = `${API_URL}${post_image_url}`;
    return (
        <Card className="mx-auto w-4/5 lg:w-[850px]">
            <CardHeader>
                <CardTitle>
                    <div className="flex flex-row items-center space-x-2">
                        <Avatar>
                            <AvatarImage src={avatarUrl} />
                            <AvatarFallback>
                                {username[0].toUpperCase()}
                            </AvatarFallback>
                        </Avatar>
                        <p>{username}</p>
                    </div>
                </CardTitle>
                <CardDescription>Posted at {postedAt}</CardDescription>
            </CardHeader>
            <CardContent>
                <p>{content}</p>
            </CardContent>
            {post_image_url && (
                <CardFooter>
                    <img
                        src={imageUrl}
                        className="rounded-md object-cover w-4/5 mx-auto"
                    />
                </CardFooter>
            )}
        </Card>
    );
};

export default PostCard;
