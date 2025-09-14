import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import { monthNames } from "@/helpers/month";
import useWithdrawStore from "@/store/withdraw/withdraw";
import { useCallback } from "react";


export default function useMonthStatusSuccessWithdrawDetail(currentMonths: number, currentYear: number, previousMonth: number, previousYear: number, toast: any) {
    const {
        findMonthStatusSuccessByCardNumber,
        monthStatusSuccess,
        loadingMonthStatusSuccess,
        setLoadingMonthStatusSuccess,
        setErrorMonthStatusSuccess,
    } = useWithdrawStore();

    const currentMonthSuccessData = monthStatusSuccess?.find(
        (balance) =>
            balance.month === monthNames[currentMonths] &&
            balance.year === currentYear.toString(),
    );

    const previousMonthSuccessData = monthStatusSuccess?.find(
        (balance) =>
            balance.month === monthNames[previousMonth] &&
            balance.year === previousYear.toString(),
    );

    const currentMonthSuccess = currentMonthSuccessData?.total_amount || 0;
    const previousMonthSuccess = previousMonthSuccessData?.total_amount || 0;

    const monthPercentageSuccessChange = calculatePercentageChange(
        currentMonthSuccess,
        previousMonthSuccess,
    );

    const fetchMonthlySuccess = useCallback(
        async (year: number, month: number,card_number: string) => {
            try {
                setLoadingMonthStatusSuccess(true);
                setErrorMonthStatusSuccess(null);

                await findMonthStatusSuccessByCardNumber(toast, year, month,card_number);
            } catch (error) {
                setErrorMonthStatusSuccess("Failed to fetch monthly balance");
            } finally {
                setLoadingMonthStatusSuccess(false);
            }
        },
        [
            findMonthStatusSuccessByCardNumber,
            setLoadingMonthStatusSuccess,
            setErrorMonthStatusSuccess,
        ],
    );

     const debouncedFetchMonthlySuccess = debounce(fetchMonthlySuccess, 300);

     return{
        monthPercentageSuccessChange,
        loadingMonthStatusSuccess,
        currentMonthSuccess,
        previousMonthSuccess,
        debouncedFetchMonthlySuccess
     }

}