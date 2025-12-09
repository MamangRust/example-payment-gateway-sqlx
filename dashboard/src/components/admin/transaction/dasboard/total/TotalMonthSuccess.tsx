import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { formatRupiah } from "@/helpers/formatRupiah";
import CardSkeleton from "@/components/ui/cardSkeleton";
import { Wallet } from "lucide-react";

export default function TotalCardMonthSuccess({ loadingMonthStatusSuccess, currentMonthSuccess, previousMonthSuccess, monthPercentageSuccessChange }: {
    loadingMonthStatusSuccess: boolean
    currentMonthSuccess: number,
    previousMonthSuccess: number,
    monthPercentageSuccessChange: number
}) {
    return (
        <>
            {loadingMonthStatusSuccess ? (
                <CardSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">
                            Month success
                        </CardTitle>
                        <Wallet className="h-6 w-6 text-gray-500" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">
                            {currentMonthSuccess
                                ? formatRupiah(currentMonthSuccess)
                                : "Data tidak tersedia"}
                        </div>
                        <p className="text-sm text-muted-foreground">
                            {previousMonthSuccess
                                ? `${monthPercentageSuccessChange >= 0 ? "↑" : "↓"} ${Math.abs(monthPercentageSuccessChange).toFixed(2)}% dari bulan sebelumnya`
                                : "Tidak ada data bulan sebelumnya"}
                        </p>
                    </CardContent>
                </Card>
            )}
        </>
    )
}