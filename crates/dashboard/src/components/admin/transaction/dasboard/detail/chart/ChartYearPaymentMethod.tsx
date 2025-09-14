import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ThemeChart from "@/components/ui/chart";
import ChartSkeleton from "@/components/ui/chartSkeleton";
import { formatRupiah } from "@/helpers/formatRupiah";


export default function ChartYearPaymentMethod({ loadingYearTransactionMethod, yearlyPaymentMethod }: { loadingYearTransactionMethod: boolean, yearlyPaymentMethod: any }) {
    return (
        <>
            {loadingYearTransactionMethod ? (
                <ChartSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader>
                        <CardTitle>Yearly Payment Method</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <ThemeChart
                            options={{
                                chart: {
                                    id: "yearly-balances-chart",
                                    toolbar: { show: false },
                                },
                                xaxis: {
                                    categories: Array.from({ length: 5 }, (_, i) =>
                                        (new Date().getFullYear() - 4 + i).toString(),
                                    ),
                                },
                                yaxis: {
                                    title: { text: "Balance (Rp)" },
                                    labels: {
                                        formatter: (value) => formatRupiah(value),
                                    },
                                },
                                tooltip: {
                                    y: {
                                        formatter: (value) => formatRupiah(value),
                                    },
                                },
                                colors: ["#6366F1", "#10B981", "#F59E0B"],
                            }}
                            series={yearlyPaymentMethod}
                            type="bar"
                            height={300}
                        />
                    </CardContent>
                </Card>
            )}
        </>
    )
}