import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import useMerchantStore from "@/store/merchant/merchant";
import { useCallback } from "react";


export default function useYearTotalAmountMerchant(currentYear: number, toast: any) {
    const {
        findYearTotalAmount,
        yearTotalAmount,
        loadingYearTotalAmount,
        setLoadingYearTotalAmount,
        setErrorYearTotalAmount,
    } = useMerchantStore();

    const currentYearData =
        yearTotalAmount && Array.isArray(yearTotalAmount)
            ? yearTotalAmount.find(
                (balance) => balance.year === currentYear.toString(),
            )
            : null;

    const previousYearData =
        yearTotalAmount && Array.isArray(yearTotalAmount)
            ? yearTotalAmount.find(
                (balance) => balance.year === (currentYear - 1).toString(),
            )
            : null;

    const currentYearBalance = currentYearData?.total_amount || 0;
    const previousYearBalance = previousYearData?.total_amount || 0;


    const yearPercentageChange = calculatePercentageChange(
        currentYearBalance,
        previousYearBalance,
    );

    const fetchYearlyTotalBalance = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTotalAmount(true);
                setErrorYearTotalAmount(null);

                await findYearTotalAmount(toast, year);
            } catch (error) {
                setErrorYearTotalAmount("Failed to fetch yearly balance");
            } finally {
                setLoadingYearTotalAmount(false);
            }
        },
        [findYearTotalAmount, setLoadingYearTotalAmount, setErrorYearTotalAmount],
    );

    const debouncedFetchYearlyTotalBalance = debounce(
        fetchYearlyTotalBalance,
        1000,
      );

    return {
        currentYearData,
        previousYearData,
        currentYearBalance,
        debouncedFetchYearlyTotalBalance,
        yearPercentageChange,
        fetchYearlyTotalBalance,
        findYearTotalAmount,
        yearTotalAmount,
        loadingYearTotalAmount,
        setLoadingYearTotalAmount,
        setErrorYearTotalAmount,
    }
}