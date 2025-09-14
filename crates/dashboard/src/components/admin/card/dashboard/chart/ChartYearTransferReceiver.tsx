

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ThemeChart from "@/components/ui/chart";
import ChartSkeleton from "@/components/ui/chartSkeleton";


export default function ChartYearTransferReceiver({ loadingYearTransferReceiver, yearlyTransfersReceiver }: { loadingYearTransferReceiver: boolean, yearlyTransfersReceiver: any }) {
    return (
        <>
            {loadingYearTransferReceiver ? (
                <ChartSkeleton />
            ) : (
                <Card className="w-full shadow-lg rounded-md border">
                    <CardHeader>
                        <CardTitle>Yearlly Transfer Receiver</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <ThemeChart
                            options={{
                                chart: {
                                    id: "Yearlly-transfer-receiver-chart",
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
                                { name: "Yearlly Transaction", data: yearlyTransfersReceiver },
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