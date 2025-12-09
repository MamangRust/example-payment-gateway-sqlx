import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ThemeChart from "@/components/ui/chart";
import ChartSkeleton from "@/components/ui/chartSkeleton";
import { formatRupiah } from "@/helpers/formatRupiah";

export default function ChartMonthAmount({ loadingMonthAmount, monthlyAmount }: { loadingMonthAmount: boolean, monthlyAmount: any }) {
    return (
        <>
            {loadingMonthAmount ? (
                <ChartSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader>
                        <CardTitle>Monthly Amount</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <ThemeChart
                            options={{
                                chart: {
                                    id: "monthly-amount-chart",
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
                            series={[{ name: "Monthly Balance", data: monthlyAmount }]}
                            type="line"
                            height={300}
                        />
                    </CardContent>
                </Card>
            )}
        </>
    )
}