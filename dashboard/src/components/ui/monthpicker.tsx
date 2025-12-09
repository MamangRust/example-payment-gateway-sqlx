import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@/components/ui/select";
import { useState } from "react";

export const months = [
  { value: "January", label: "Januari", number: 1 },
  { value: "February", label: "Februari", number: 2 },
  { value: "March", label: "Maret", number: 3 },
  { value: "April", label: "April", number: 4 },
  { value: "May", label: "Mei", number: 5 },
  { value: "June", label: "Juni", number: 6 },
  { value: "July", label: "Juli", number: 7 },
  { value: "August", label: "Agustus", number: 8 },
  { value: "September", label: "September", number: 9 },
  { value: "October", label: "Oktober", number: 10 },
  { value: "November", label: "November", number: 11 },
  { value: "December", label: "Desember", number: 12 },
];

interface MonthPickerProps {
  onMonthChange: (month: { name: string; number: number }) => void;
  initialMonth?: number;
}

export function MonthPicker({
  onMonthChange,
  initialMonth = new Date().getMonth() + 1,
}: MonthPickerProps) {
  const initialMonthName =
    months.find((m) => m.number === initialMonth)?.value || "January";
  const [selectedMonth, setSelectedMonth] = useState<string>(initialMonthName);

  const handleMonthChange = (value: string) => {
    setSelectedMonth(value);

    const selected = months.find((month) => month.value === value);
    if (selected) {
      onMonthChange({ name: selected.value, number: selected.number }); // Kirim nama dan angka bulan
    }
  };

  return (
    <div>
      <Select value={selectedMonth} onValueChange={handleMonthChange}>
        <SelectTrigger className="w-[180px]">
          <SelectValue placeholder="Pilih Bulan" />
        </SelectTrigger>
        <SelectContent>
          {months.map((month) => (
            <SelectItem key={month.value} value={month.value}>
              {month.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}
