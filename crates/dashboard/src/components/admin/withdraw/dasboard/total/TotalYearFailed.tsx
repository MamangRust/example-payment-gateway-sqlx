import { TopupYearStatusFailed } from "@/types/model"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { formatRupiah } from "@/helpers/formatRupiah";
import CardSkeleton from "@/components/ui/cardSkeleton";
import { TrendingDown } from "lucide-react";

export default function TotalCardYearFailed({
    loadingYearStatusFailed,
    currentYearFailed,
    previousYearFailedData,
    yearPercentageFailedChange
}: {
    loadingYearStatusFailed: boolean,
    currentYearFailed: number,
    previousYearFailedData: TopupYearStatusFailed,
    yearPercentageFailedChange: number
}) {
    return (
        <>
            {loadingYearStatusFailed ? (
                <CardSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">Year Failed</CardTitle>
                        <TrendingDown className="h-6 w-6 text-gray-500" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">
                            {currentYearFailed
                                ? formatRupiah(currentYearFailed)
                                : "Data tidak tersedia"}
                        </div>
                        {currentYearFailed && previousYearFailedData ? (
                            <p className="text-sm text-muted-foreground">
                                {yearPercentageFailedChange >= 0 ? "↑" : "↓"}{" "}
                                {Math.abs(yearPercentageFailedChange).toFixed(2)}% dari tahun
                                sebelumnya
                            </p>
                        ) : (
                            <p className="text-sm text-muted-foreground">
                                Tidak ada data tahun sebelumnya
                            </p>
                        )}
                    </CardContent>
                </Card>
            )}
        </>
    )
}