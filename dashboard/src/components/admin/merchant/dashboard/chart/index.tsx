import ChartMonthAmount from "./ChartMonthAmount";
import ChartMonthPaymentMethod from "./ChartMonthPaymentMethod";
import ChartYearAmount from "./ChartYearAmount";
import ChartYearPaymentMethod from "./ChartYearPaymentMethod";

interface ChartDashboardMerchantProps {
    loadingMonthAmount: boolean;
    loadingMonthPaymentMethod: boolean;
    loadingYearAmount: boolean;
    loadingYearPaymentMethod: boolean

    monthlyAmount: any;
    monthlyPaymentMethod: any;
    yearlyAmount: any;
    yearlyPaymentMethod: any;
}

export default function ChartDashboarMerchant({
    loadingMonthAmount,
    loadingMonthPaymentMethod,
    loadingYearAmount,
    loadingYearPaymentMethod,
    monthlyAmount,
    monthlyPaymentMethod,
    yearlyAmount,
    yearlyPaymentMethod
}: ChartDashboardMerchantProps) {

    return (
        <>
            <ChartMonthPaymentMethod loadingMonthPaymentMethod={loadingMonthPaymentMethod} monthlyPaymentMethod={monthlyPaymentMethod} />
            <ChartYearPaymentMethod loadingYearPaymentMethod={loadingYearPaymentMethod} yearlyPaymentMethod={yearlyPaymentMethod} />
            <ChartMonthAmount loadingMonthAmount={loadingMonthAmount} monthlyAmount={monthlyAmount} />
            <ChartYearAmount loadingYearAmount={loadingYearAmount} yearlyAmount={yearlyAmount} />
        </>
    )
}