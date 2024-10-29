import CreatePostForm from './components/CreatePostForm';
import PostsList from './components/PostsList';

function App() {
    return (
        <div className="w-full h-screen">
            <div className="mx-auto w-4/5 lg:w-[650px]">
                <CreatePostForm />
                <PostsList />
            </div>
        </div>
    );
}

export default App;
