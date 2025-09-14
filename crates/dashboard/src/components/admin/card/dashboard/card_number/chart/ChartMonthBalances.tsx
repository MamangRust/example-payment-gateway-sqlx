import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ThemeChart from "@/components/ui/chart";
import ChartSkeleton from "@/components/ui/chartSkeleton";
import { formatRupiah } from "@/helpers/formatRupiah";


export default function CharthMonthBalances({ loadingMonthBalance, monthlyBalances }: {
    loadingMonthBalance: boolean, monthlyBalances: any
}) {
    return (
        <>
            {loadingMonthBalance ? (
                <ChartSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader>
                        <CardTitle>Monthly Balances</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <ThemeChart
                            options={{
                                chart: {
                                    id: "monthly-balances-chart",
                                    toolbar: { show: false },
                                },
                                xaxis: {
                                    categories: [
                                        "Jan",
                                        "Feb",
                                        "Mar",
                                        "Apr",
                                        "May",
                                        "Jun",
                                        "Jul",
                                        "Aug",
                                        "Sep",
                                        "Oct",
                                        "Nov",
                                        "Dec",
                                    ],
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
                                colors: ["#6366F1"],
                            }}
                            series={[{ name: "Monthly Balance", data: monthlyBalances }]}
                            type="line"
                            height={300}
                        />
                    </CardContent>
                </Card>
            )}
        </>
    )
}