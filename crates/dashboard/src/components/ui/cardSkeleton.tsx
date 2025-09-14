import { Skeleton } from "./skeleton";

export default function CardSkeleton() {
  return (
    <div className="w-full shadow-lg rounded-md border p-4">
      <div className="flex flex-row items-center justify-between pb-2">
        <Skeleton className="h-4 w-24" />
        <Skeleton className="h-6 w-6 rounded-full" />
      </div>
      <div className="pt-2">
        <Skeleton className="h-8 w-32" />
      </div>
    </div>
  );
}
