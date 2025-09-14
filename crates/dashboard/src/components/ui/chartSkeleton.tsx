import { Skeleton } from "./skeleton";

export default function ChartSkeleton() {
  return (
    <div className="w-full shadow-lg rounded-md border p-4">
      <div className="pb-4">
        <Skeleton className="h-6 w-32" />
      </div>
      <div className="space-y-2">
        <Skeleton className="h-4 w-full" />
        <Skeleton className="h-4 w-full" />
        <Skeleton className="h-4 w-full" />
        <Skeleton className="h-4 w-full" />
        <Skeleton className="h-4 w-full" />
      </div>
    </div>
  );
}
