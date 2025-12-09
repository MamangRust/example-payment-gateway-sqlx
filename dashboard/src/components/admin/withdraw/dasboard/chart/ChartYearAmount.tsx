import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ThemeChart from "@/components/ui/chart";
import ChartSkeleton from "@/components/ui/chartSkeleton";
import { formatRupiah } from "@/helpers/formatRupiah";

export default function ChartYearAmount({ loadingYearWithdrawAmount, yearlyAmount }: { loadingYearWithdrawAmount: boolean, yearlyAmount: any }) {
    return (
        <>
            {loadingYearWithdrawAmount ? (
                <ChartSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader>
                        <CardTitle>Yearly Balances</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <ThemeChart
                            options={{
                                chart: {
                                    id: "yearly-amount-chart",
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
                                colors: ["#22C55E"],
                            }}
                            series={[{ name: "Yearly Amount", data: yearlyAmount }]}
                            type="bar"
                            height={300}
                        />
                    </CardContent>
                </Card>
            )}
        </>
    )
}