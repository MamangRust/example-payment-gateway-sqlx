import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ThemeChart from "@/components/ui/chart";
import ChartSkeleton from "@/components/ui/chartSkeleton";
import { formatRupiah } from "@/helpers/formatRupiah";


export default function ChartMonthPaymentMethodDetail({ loadingMonthPaymentMethod, monthlyPaymentMethod }: { loadingMonthPaymentMethod: boolean, monthlyPaymentMethod: any }) {
    return (
        <>
            {loadingMonthPaymentMethod ? (
                <ChartSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader>
                        <CardTitle>Monthly Payment Method</CardTitle>
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
                                colors: ["#6366F1", "#10B981", "#F59E0B"],
                            }}
                            series={monthlyPaymentMethod}
                            type="line"
                            height={300}
                        />
                    </CardContent>
                </Card>
            )}
        </>
    )
}