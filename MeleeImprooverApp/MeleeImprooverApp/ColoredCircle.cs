using System.ComponentModel;

namespace MeleeImprooverApp;

// Copilot slop but it works
[ToolboxItem(true)]
public class ColoredCircle : Control
{
    private Color _circleColor = Color.Red;

    public ColoredCircle()
    {
        this.Size = new Size(20, 20);
    }

    public void SetColor(Color color)
    {
        _circleColor = color;
        this.Invalidate(); // Trigger repaint
    }

    protected override void OnPaint(PaintEventArgs e)
    {
        base.OnPaint(e);
        using (Brush brush = new SolidBrush(_circleColor))
        {
            e.Graphics.SmoothingMode = System.Drawing.Drawing2D.SmoothingMode.AntiAlias;
            e.Graphics.FillEllipse(brush, 0, 0, this.Width - 1, this.Height - 1);
        }
    }

    protected override void OnResize(EventArgs e)
    {
        base.OnResize(e);
        this.Invalidate(); // Redraw on resize
    }
}