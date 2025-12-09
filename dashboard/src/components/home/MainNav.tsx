import { Link, useLocation } from 'react-router-dom';
import { cn } from '@/lib/utils';

export function MainNav() {
  const location = useLocation();
  const pathname = location.pathname;

  return (
    <nav className="hidden md:flex items-center space-x-4 lg:space-x-6 ml-4">
      <div className="flex-1 flex justify-start">
        <Link to="/" className="mr-6 flex items-center space-x-2">
          <span className="hidden font-bold sm:inline-block">SanEdge</span>
        </Link>
      </div>
      <div className="flex items-center space-x-4">
        <Link
          to="#services"
          className={cn(
            'text-sm font-medium transition-colors hover:text-primary',
            pathname === '#services' ? 'text-foreground' : 'text-foreground/60'
          )}
        >
          Services
        </Link>
        <Link
          to="#about"
          className={cn(
            'text-sm font-medium transition-colors hover:text-primary',
            pathname === '#about' ? 'text-foreground' : 'text-foreground/60'
          )}
        >
          About
        </Link>
        <Link
          to="#team"
          className={cn(
            'text-sm font-medium transition-colors hover:text-primary',
            pathname === '#team' ? 'text-foreground' : 'text-foreground/60'
          )}
        >
          Team
        </Link>
        <Link
          to="#contact"
          className={cn(
            'text-sm font-medium transition-colors hover:text-primary',
            pathname === '#contact' ? 'text-foreground' : 'text-foreground/60'
          )}
        >
          Contact
        </Link>
      </div>
    </nav>
  );
}
