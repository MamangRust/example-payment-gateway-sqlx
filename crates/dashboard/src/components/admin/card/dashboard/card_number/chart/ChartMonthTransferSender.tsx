import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ThemeChart from "@/components/ui/chart";
import ChartSkeleton from "@/components/ui/chartSkeleton";


export default function ChartMonthTransferSender({ loadingMonthTransferSender, monthlyTransfersSender }: { loadingMonthTransferSender: boolean, monthlyTransfersSender: any }) {
    return (
        <>
            {loadingMonthTransferSender ? (
                <ChartSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader>
                        <CardTitle>Monthly Transfers Sender</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <ThemeChart
                            options={{
                                chart: {
                                    id: "monthly-transfer-sender-chart",
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
                                yaxis: { title: { text: "Transaction ($)" } },
                                colors: ["#10B981"],
                            }}
                            series={[
                                { name: "Monthly Transaction", data: monthlyTransfersSender },
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