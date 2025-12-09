import { TopupYearStatusSuccess } from "@/types/model"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { formatRupiah } from "@/helpers/formatRupiah";
import CardSkeleton from "@/components/ui/cardSkeleton";
import { TrendingDown } from "lucide-react";

export default function TotalCardYearSuccess({
    loadingYearStatusSuccess,
    currentYearSuccess,
    previousYearSuccessData,
    yearPercentageSuccessChange
}: {
    loadingYearStatusSuccess: boolean,
    currentYearSuccess: number,
    previousYearSuccessData: TopupYearStatusSuccess,
    yearPercentageSuccessChange: number
}) {
    return (
        <>
            {loadingYearStatusSuccess ? (
                <CardSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">
                            Year Success
                        </CardTitle>
                        <TrendingDown className="h-6 w-6 text-gray-500" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">
                            {currentYearSuccess
                                ? formatRupiah(currentYearSuccess)
                                : "Data tidak tersedia"}
                        </div>
                        {currentYearSuccess && previousYearSuccessData ? (
                            <p className="text-sm text-muted-foreground">
                                {yearPercentageSuccessChange >= 0 ? "↑" : "↓"}{" "}
                                {Math.abs(yearPercentageSuccessChange).toFixed(2)}% dari tahun
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