import { Link, useLocation } from "react-router-dom";
import { useEffect } from "react";
import { Home } from "lucide-react";
import { Button } from "@/components/ui/button";

const NotFound = () => {
  const location = useLocation();

  useEffect(() => {
    console.error("404 Error: User attempted to access non-existent route:", location.pathname);
  }, [location.pathname]);

  return (
    <div className="container min-h-[60vh] flex items-center justify-center">
      <div className="text-center max-w-md">
        <div className="text-7xl md:text-9xl font-black text-gradient leading-none">404</div>
        <h1 className="mt-6 text-2xl md:text-3xl font-bold">Page not found</h1>
        <p className="mt-3 text-muted-foreground">
          The page <code className="text-foreground">{location.pathname}</code> doesn't exist on this Samaris site.
        </p>
        <Button asChild className="mt-8 bg-gradient-primary text-primary-foreground border-0 hover:opacity-90">
          <Link to="/"><Home className="mr-2 size-4" />Back to home</Link>
        </Button>
      </div>
    </div>
  );
};

export default NotFound;
