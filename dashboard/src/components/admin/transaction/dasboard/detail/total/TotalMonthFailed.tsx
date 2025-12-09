import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { formatRupiah } from "@/helpers/formatRupiah";
import CardSkeleton from "@/components/ui/cardSkeleton";
import { Wallet } from "lucide-react";

export default function TotalCardMonthFailed({ loadingMonthStatusFailed, currentMonthFailed, previousMonthFailed, monthPercentageFailedChange }: {
    loadingMonthStatusFailed: boolean
    currentMonthFailed: number,
    previousMonthFailed: number,
    monthPercentageFailedChange: number
}) {
    return (
        <>
            {loadingMonthStatusFailed ? (
                <CardSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">
                            Total Balance Month
                        </CardTitle>
                        <Wallet className="h-6 w-6 text-gray-500" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">
                            {currentMonthFailed
                                ? formatRupiah(currentMonthFailed)
                                : "Data tidak tersedia"}
                        </div>
                        <p className="text-sm text-muted-foreground">
                            {previousMonthFailed
                                ? `${monthPercentageFailedChange >= 0 ? "↑" : "↓"} ${Math.abs(monthPercentageFailedChange).toFixed(2)}% dari bulan sebelumnya`
                                : "Tidak ada data bulan sebelumnya"}
                        </p>
                    </CardContent>
                </Card>
            )}
        </>
    )
}