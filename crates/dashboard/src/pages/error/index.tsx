import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AlertCircle, ArrowLeft, RefreshCcw } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { useToast } from '@/hooks/use-toast';

export default function ErrorPage() {
  const navigate = useNavigate();
  const { toast } = useToast();
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    setError(new Error('An unexpected error occurred'));
  }, []);

  const handleRefresh = () => {
    window.location.reload();
  };

  const handleBack = () => {
    navigate(-1);
  };

  const copyErrorMessage = () => {
    if (error) {
      navigator.clipboard.writeText(error.message);
      toast({
        description: 'Error message copied to clipboard',
      });
    }
  };

  return (
    <div className="flex items-center justify-center min-h-screen bg-background p-4">
      <Card className="w-full max-w-md">
        <CardHeader>
          <div className="flex items-center space-x-2">
            <AlertCircle className="h-6 w-6 text-destructive" />
            <CardTitle className="text-2xl font-bold">Oops!</CardTitle>
          </div>
          <CardDescription>
            Sorry, an unexpected error has occurred.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-sm font-medium mb-2">
            The page you tried to open has the following error:
          </p>
          <pre className="bg-muted p-2 rounded-md text-sm overflow-x-auto">
            {error ? error.message : 'Unknown error'}
          </pre>
        </CardContent>
        <CardFooter className="flex flex-col space-y-2 sm:flex-row sm:space-x-2 sm:space-y-0">
          <Button onClick={handleBack} className="w-full sm:w-auto">
            <ArrowLeft className="mr-2 h-4 w-4" /> Go Back
          </Button>
          <Button
            onClick={handleRefresh}
            variant="outline"
            className="w-full sm:w-auto"
          >
            <RefreshCcw className="mr-2 h-4 w-4" /> Refresh
          </Button>
          <Button
            onClick={copyErrorMessage}
            variant="secondary"
            className="w-full sm:w-auto"
          >
            Copy Error
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}
