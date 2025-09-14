import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ThemeChart from "@/components/ui/chart";
import ChartSkeleton from "@/components/ui/chartSkeleton";


export default function ChartYearWithdrawAmount({ loadingYearWithdrawAmount, yearlyWithdraws }: { loadingYearWithdrawAmount: boolean, yearlyWithdraws: any }) {
    return (
        <>
            {loadingYearWithdrawAmount ? (
                <ChartSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader>
                        <CardTitle>Yearly Withdraws</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <ThemeChart
                            options={{
                                chart: {
                                    id: "Yearly-withdraws-chart",
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
                                yaxis: { title: { text: "Withdraw ($)" } },
                                colors: ["#EF4444"],
                            }}
                            series={[
                                { name: "Yearly Withdraw", data: yearlyWithdraws },
                            ]}
                            type="line"
                            height={300}
                        />
                    </CardContent>
                </Card>
            )}
        </>
    )

}