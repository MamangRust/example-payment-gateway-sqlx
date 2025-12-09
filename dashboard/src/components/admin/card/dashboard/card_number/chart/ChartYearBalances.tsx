import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ThemeChart from "@/components/ui/chart";
import ChartSkeleton from "@/components/ui/chartSkeleton";
import { formatRupiah } from "@/helpers/formatRupiah";

export default function ChartYearBalances({ loadingYearBalance, yearlyBalances }: { loadingYearBalance: boolean, yearlyBalances: any }) {
    return (
        <>
            {loadingYearBalance ? (
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
                                colors: ["#22C55E"],
                            }}
                            series={[{ name: "Yearly Balance", data: yearlyBalances }]}
                            type="bar"
                            height={300}
                        />
                    </CardContent>
                </Card>
            )}
        </>
    )
}