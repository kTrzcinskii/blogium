import CreatePostForm from './components/CreatePostForm';
import PostsList from './components/PostsList';

function App() {
    return (
        <div className="w-full h-screen">
            <CreatePostForm />
            <PostsList />
        </div>
    );
}

export default App;
