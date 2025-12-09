import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import useMerchantStore from "@/store/merchant/merchant";
import { useCallback } from "react";


export default function useYearTotalAmountMerchantDetail(currentYear: number, toast: any) {
    const {
        findYearTotalAmountByMerchant,
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
        async (year: number, merchant_id: number) => {
            try {
                setLoadingYearTotalAmount(true);
                setErrorYearTotalAmount(null);

                await findYearTotalAmountByMerchant(toast, year, merchant_id);
            } catch (error) {
                setErrorYearTotalAmount("Failed to fetch yearly balance");
            } finally {
                setLoadingYearTotalAmount(false);
            }
        },
        [findYearTotalAmountByMerchant, setLoadingYearTotalAmount, setErrorYearTotalAmount],
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
        findYearTotalAmountByMerchant,
        yearTotalAmount,
        loadingYearTotalAmount,
        setLoadingYearTotalAmount,
        setErrorYearTotalAmount,
    }
}