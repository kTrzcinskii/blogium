import { useNavigate } from 'react-router';
import { Button } from './ui/button';

const NotFound = () => {
    const navigate = useNavigate();

    return (
        <div className="w-full h-screen flex justify-center items-center flex-col">
            <h1 className="text-5xl font-semibold mb-3">404</h1>
            <p className="mb-8">Looks like this link is incorrect.</p>
            <Button onClick={() => navigate('/home')}>Go to home page</Button>
        </div>
    );
};

export default NotFound;
