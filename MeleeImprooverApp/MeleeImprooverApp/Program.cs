using System.Runtime.InteropServices;

namespace MeleeImprooverApp;

internal static class Program
{
    [DllImport("slippi_analyzer.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void test_replay(IntPtr name, Int32 pChar1, Int32 pChar2);

    //[DllImport("slippi_analyzer.dll")]
    [DllImport("slippi_analyzer.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr read_info(IntPtr namePtr);

    [DllImport("slippi_analyzer.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_info(IntPtr ptr);

    [DllImport("slippi_analyzer.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr check_clips(IntPtr namePtr, out UIntPtr length, int hmnPlayer, int saveButtonType);

    [DllImport("slippi_analyzer.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_clip_array(IntPtr ptr, UIntPtr length);

    [DllImport("slippi_analyzer.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void create_savestates(IntPtr exportPathPtr, IntPtr fileNamePtr, IntPtr gameNamePtr, int hmnPlayer,
        int clipLength, int namingStyle, int saveButtonType, int startFrame);

    [DllImport("slippi_analyzer.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void create_filtered_savestates(IntPtr exportPathPtr, IntPtr fileNamePtr, IntPtr gameNamePtr, Int32 hmnPlayer,
        Int32 clipLength);

    /// <summary>
    ///  The main entry point for the application.
    /// </summary>
    [STAThread]
    internal static void Main()
    {
        // EnableVisualStyles
        // SetCompatibleTextRenderingDefault(false)
        // SetHighDpiMode(SystemAware)
        ApplicationConfiguration.Initialize();
        Application.Run(new Form1());
    }
}