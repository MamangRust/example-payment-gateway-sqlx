import { useState } from "react";
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet";
import { Button } from "@/components/ui/button";
import { Menu } from "lucide-react";
import { Link, useNavigate } from "react-router-dom";

export function MobileNav() {
  const [open, setOpen] = useState(false);

  return (
    <Sheet open={open} onOpenChange={setOpen}>
      <SheetTrigger asChild>
        <Button variant="outline" className="w-10 px-0 sm:hidden"> 
          <Menu className="h-5 w-5" />
          <span className="sr-only">Open Mobile Menu</span>
        </Button>
      </SheetTrigger>
      <SheetContent side="right">
        <MobileLink onOpenChange={setOpen} to="/" className="flex items-center">
          <span className="font-bold">SanEdge</span>
        </MobileLink>
        <div className="flex flex-col gap-3 mt-3">
          
          <MobileLink onOpenChange={setOpen} to="#services">
            Services
          </MobileLink>
          <MobileLink onOpenChange={setOpen} to="#about">
            About
          </MobileLink>
          <MobileLink onOpenChange={setOpen} to="#team">
            Team
          </MobileLink>
          <MobileLink onOpenChange={setOpen} to="#contact">
            Contact
          </MobileLink>
        </div>
      </SheetContent>
    </Sheet>
  );
}

interface MobileLinkProps {
  children: React.ReactNode;
  onOpenChange?: (open: boolean) => void;
  to: string;
  className?: string;
}

function MobileLink({ to, onOpenChange, children, className }: MobileLinkProps) {
  const navigate = useNavigate();

  return (
    <Link
      to={to}
      onClick={() => {
        navigate(to);
        onOpenChange?.(false);
      }}
      className={className}
    >
      {children}
    </Link>
  );
}
