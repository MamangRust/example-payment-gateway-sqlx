import { MerchantMonthlyTotalAmount } from "@/types/model";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { formatRupiah } from "@/helpers/formatRupiah";
import CardSkeleton from "@/components/ui/cardSkeleton";
import { Wallet } from "lucide-react";

export default function TotalCardMonthAmountDetail({ loadingMonthTotalAmount, currentMonthData, currentMonthBalance, previousMonthData, monthPercentageChange }: {
    loadingMonthTotalAmount: boolean
    currentMonthData: MerchantMonthlyTotalAmount,
    currentMonthBalance: number,
    previousMonthData: MerchantMonthlyTotalAmount,
    monthPercentageChange: number
}) {
    return (
        <>
            {loadingMonthTotalAmount ? (
                <CardSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">
                            Total Amount Month
                        </CardTitle>
                        <Wallet className="h-6 w-6 text-gray-500" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">
                            {currentMonthData
                                ? formatRupiah(currentMonthBalance)
                                : "Data tidak tersedia"}
                        </div>
                        <p className="text-sm text-muted-foreground">
                            {previousMonthData
                                ? `${monthPercentageChange >= 0 ? "↑" : "↓"} ${Math.abs(monthPercentageChange).toFixed(2)}% dari bulan sebelumnya`
                                : "Tidak ada data bulan sebelumnya"}
                        </p>
                    </CardContent>
                </Card>
            )}
        </>
    )
}