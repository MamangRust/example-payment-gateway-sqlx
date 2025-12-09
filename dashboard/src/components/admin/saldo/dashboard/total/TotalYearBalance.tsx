import { SaldoYearTotalBalance } from "@/types/model"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { formatRupiah } from "@/helpers/formatRupiah";
import CardSkeleton from "@/components/ui/cardSkeleton";
import { TrendingDown } from "lucide-react";

export default function TotalCardYearBalance({
    loadingYearTotalBalance,
    currentYearData,
    currentYearBalance,
    previousYearData,
    yearPercentageChange
}: {
    loadingYearTotalBalance: boolean,
    currentYearData: SaldoYearTotalBalance,
    currentYearBalance: number,
    previousYearData: SaldoYearTotalBalance,
    yearPercentageChange: number
}) {
    return (
        <>
            {loadingYearTotalBalance ? (
                <CardSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">
                            Total Balance Year
                        </CardTitle>
                        <TrendingDown className="h-6 w-6 text-gray-500" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">
                            {currentYearData
                                ? formatRupiah(currentYearBalance)
                                : "Data tidak tersedia"}
                        </div>
                        {currentYearData && previousYearData ? (
                            <p className="text-sm text-muted-foreground">
                                {yearPercentageChange >= 0 ? "↑" : "↓"}{" "}
                                {Math.abs(yearPercentageChange).toFixed(2)}% dari tahun
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