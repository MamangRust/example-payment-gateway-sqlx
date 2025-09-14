import ChartMonthAmountDetail from "./ChartMonthAmount";
import ChartMonthPaymentMethodDetail from "./ChartMonthPaymentMethod";
import ChartYearAmountDetail from "./ChartYearAmount";
import ChartYearPaymentMethodDetail from "./ChartYearPaymentMethod";

interface ChartDashboardMerchanDetailtProps {
    loadingMonthAmount: boolean;
    loadingMonthPaymentMethod: boolean;
    loadingYearAmount: boolean;
    loadingYearPaymentMethod: boolean

    monthlyAmount: any;
    monthlyPaymentMethod: any;
    yearlyAmount: any;
    yearlyPaymentMethod: any;
}

export default function ChartDashboarMerchantDetail({
    loadingMonthAmount,
    loadingMonthPaymentMethod,
    loadingYearAmount,
    loadingYearPaymentMethod,
    monthlyAmount,
    monthlyPaymentMethod,
    yearlyAmount,
    yearlyPaymentMethod
}: ChartDashboardMerchanDetailtProps) {

    return (
        <>
            <ChartMonthPaymentMethodDetail loadingMonthPaymentMethod={loadingMonthPaymentMethod} monthlyPaymentMethod={monthlyPaymentMethod} />
            <ChartYearPaymentMethodDetail loadingYearPaymentMethod={loadingYearPaymentMethod} yearlyPaymentMethod={yearlyPaymentMethod} />
            <ChartMonthAmountDetail loadingMonthAmount={loadingMonthAmount} monthlyAmount={monthlyAmount} />
            <ChartYearAmountDetail loadingYearAmount={loadingYearAmount} yearlyAmount={yearlyAmount} />
        </>
    )
}