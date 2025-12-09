import { Button } from "@/components/ui/button";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { ChevronDown } from "lucide-react";
import { useState } from "react";

interface YearPickerProps {
  selectedYear: number;
  onYearChange: (year: number) => void;
}

export function YearPicker({ selectedYear, onYearChange }: YearPickerProps) {
  const [isOpen, setIsOpen] = useState(false);

  const startYear = 1900;
  const endYear = 2100;
  const years = Array.from(
    { length: endYear - startYear + 1 },
    (_, i) => startYear + i,
  );

  return (
    <Popover open={isOpen} onOpenChange={setIsOpen}>
      <PopoverTrigger asChild>
        <Button variant="outline">
          {selectedYear}
          <ChevronDown className="ml-2 h-4 w-4" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-auto p-2 max-h-60 overflow-y-auto">
        <div className="grid grid-cols-3 gap-2">
          {years.map((year) => (
            <Button
              key={year}
              variant="ghost"
              onClick={() => {
                onYearChange(year);
                setIsOpen(false);
              }}
            >
              {year}
            </Button>
          ))}
        </div>
      </PopoverContent>
    </Popover>
  );
}
