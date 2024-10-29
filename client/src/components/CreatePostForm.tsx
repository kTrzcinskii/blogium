import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import * as z from 'zod';
import { Input } from './ui/input';
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from './ui/form';
import { Button } from './ui/button';
import { Textarea } from './ui/textarea';
import { useCreatePostMutation } from '@/api/mutations/useCreatePostMutation';

const CreatePostInputSchema = z.object({
    username: z.string().min(1, 'Username is required'),
    content: z.string().min(1, 'Post content is required'),
    image: z.instanceof(FileList).optional(),
    avatarUrl: z.string().optional(),
});

export type ICreatePostInput = z.infer<typeof CreatePostInputSchema>;

const CreatePostForm = () => {
    const form = useForm<ICreatePostInput>({
        resolver: zodResolver(CreatePostInputSchema),
        defaultValues: {
            username: '',
            content: '',
            image: undefined,
            avatarUrl: '',
        },
    });

    // TODO: invalidate query when post is created
    const { mutate } = useCreatePostMutation();

    const imageRef = form.register('image');

    const onSubmit = (data: ICreatePostInput) => {
        mutate(data, {
            // TODO: remove this console logs
            onSuccess: (data) => {
                console.log(data);
                form.reset();
            },
            onError: (err) => {
                console.log(err);
            },
        });
    };

    return (
        <Form {...form}>
            <form
                onSubmit={form.handleSubmit(onSubmit)}
                className="mx-auto w-4/5 lg:w-[700px] space-y-6"
            >
                <FormField
                    control={form.control}
                    name="username"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Username</FormLabel>
                            <FormControl>
                                <Input {...field} />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="content"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Content</FormLabel>
                            <FormControl>
                                <Textarea
                                    className="h-40 resize-none"
                                    {...field}
                                />
                            </FormControl>
                            <FormDescription>
                                You can use markdown syntax, e.g.{' '}
                                <span>{'# Title'}</span>,{' '}
                                <span>
                                    {'[Github](https://www.github.com)'}
                                </span>
                                .
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="image"
                    render={() => (
                        <FormItem>
                            <FormLabel>Image</FormLabel>
                            <FormControl>
                                <Input
                                    type="file"
                                    {...imageRef}
                                    accept="image/*"
                                />
                            </FormControl>
                            <FormDescription>
                                Upload image to make your post even better!
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="avatarUrl"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Avatar</FormLabel>
                            <FormControl>
                                <Input
                                    placeholder="Enter url pointing to your avatar..."
                                    {...field}
                                />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <Button type="submit">Create Post</Button>
            </form>
        </Form>
    );
};

export default CreatePostForm;
